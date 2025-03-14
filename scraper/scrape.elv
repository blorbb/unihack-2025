#!/usr/bin/env elvish

fn chunked-peach {|size f items|
	chunk $size $items | each {|xs| peach $f $xs }
}

fn get-units {
	fn units {|from|
		# 2025, Clayton, S1
		xhs api-ap-southeast-2.prod.courseloop.com/publisher/search-academic-items --raw '{"siteId":"monash-prod-pres","query":"","contenttype":"subject","searchFilters":[{"filterField":"implementationYear","filterValue":["2025"],"isExactMatch":false},{"filterField":"location","filterValue":["37f321e1db30a7406add773c3496192e"],"isExactMatch":false},{"filterField":"teachingPeriod","filterValue":["0da429a1db30a7406add773c3496190c"],"isExactMatch":false}],"from":'$from',"size":"100"}'
	}

	mkdir -p data/units
	var total = 1198
	range (/ $total 100) ^
	| peach {|x|
		units (* $x 100) ^
		| from-json ^
		| all (one)[data][results] ^
		| each {|x| put $x[code] } ^
		| to-lines ^
		> data/units/$x.txt
	}
	cat (range (/ $total 100) | each {|x| put data/units/$x.txt }) > data/units.txt
	rm -rf data/units
}

fn get-classes {|file|
	fn _classes {|unit mode|
		xhs -If will.io/timetables/activitydata.php unitCode=$unit'_CL_S1_'$mode
	}

	var modes = [ON-CAMPUS FLEXIBLE ONLINE BLENDED ON-BLK IMMERS-BLK]

	fn classes {|unit|
		var result
		for mode $modes {
			try {
				set result = (_classes $unit $mode | slurp)
			} catch {
				continue
			}
			break
		}

		if (eq $result $nil) {
			echo skipping $unit
		} else {
			echo $result > data/classes/$unit.json
		}
	}

	mkdir -p data/classes
	var @units = (from-lines < $file)
	chunked-peach 50 {|x| classes $x } $units
}

# get-classes data/units.txt

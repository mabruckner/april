#include "apriltag-2016-12-01/common/zarray.h"
#include "apriltag-2016-12-01/apriltag.h"
#include "apriltag-2016-12-01/tag16h5.h"
#include "apriltag-2016-12-01/tag25h7.h"
#include "apriltag-2016-12-01/tag25h9.h"
#include "apriltag-2016-12-01/tag36h10.h"
#include "apriltag-2016-12-01/tag36h11.h"

void b_zarray_get(const zarray_t *za, int idx, void *p) {
    zarray_get(za, idx, p);
}


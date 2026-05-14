//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1345/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1345<F: Float>(t11598: F, t11608: F, t11613: F, t11616: F, t11620: F, t11621: F, t11625: F, t11639: F, t11640: F, t11868: F, t11869: F, t11872: F, t11877: F, t11888: F, t11889: F, t11890: F, t11904: F, t11907: F, t11910: F, t11914: F, t11918: F, t11919: F, t11925: F, t11928: F, t11935: F, t1201: F, t1215: F, t1235: F, t1238: F, t1241: F, t1244: F, t1246: F, t1249: F, t1251: F, t1252: F, t15429: F, t3481: F, t3487: F, t3493: F, t3565: F, t3590: F, t3593: F, t3598: F, t3600: F, t3604: F, t3620: F, t3621: F, t3624: F, t3625: F, t3628: F, t3631: F, t44412: F, t44657: F, t44662: F, t44669: F, t44673: F, t44691: F, t44698: F, t44700: F, t44701: F, t44748: F, t44753: F, t44754: F, t44774: F, t45332: F, t491: F, t494: F, t498: F, t5079: F) -> (F,) {
    let t45344 = 6.0 * t3481 * t3590 * t498 + 24.0 * t3593 * t11935 - 6.0 * t11928 * t3631 + 12.0 * t11925 * t3600 - 24.0 * t3593 * t11608 - 4.0 * t44412 * t1252 + t44657 * t491 * t498 + 24.0 * t11613 * t3600 + 6.0 * t1238 * t3598 * t44662 - 4.0 * t3487 * t11919 - t1238 * t1241 * (6.0 * t1244 * t3590 * t3493 * t1246 - 4.0 * t3624 * t11639 * t5079 - 24.0 * t11888 * t44673 * t11889 - 3.0 * t3624 * t44669 * t3625 - 36.0 * t44698 * t44700 * t44701 + 4.0 * t11616 * t1249 - 12.0 * t11907 * t11625 + 4.0 * t3604 * t11640 + 6.0 * t11877 * t3621 - 24.0 * t44691 * t11890 + t44748 + 4.0 * t1244 * t11868 * t1215 * t1246 - 12.0 * t3624 * t11620 * t5079 + 6.0 * t11914 * t3620 * t15429 + 14.0 * t44753 * t44700 * t44754 + 12.0 * t3604 * t11621 + 4.0 * t1201 * t11869 + 24.0 * t11904 * t11872 - 12.0 * t11907 * t11910 + 6.0 * t3565 * t3628 + t44774 * t494 + t45332) + 4.0 * t11598 * t1235 * t498 + 8.0 * t1238 * t3598 * t11918 * t1251;
    (t45344,)
}

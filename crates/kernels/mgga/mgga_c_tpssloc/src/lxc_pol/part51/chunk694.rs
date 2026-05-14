//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 694/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk694<F: Float>(t1351: F, t562: F, t550: F, t6976: F, t1992: F, t1372: F, t1998: F, t214: F, t1985: F, t1338: F, t2006: F, t1352: F, t553: F, t6955: F, t1332: F, t1336: F, t2013: F, t544: F, t6967: F, t6971: F, t6975: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6977 = t562 * t1351;
    let t6978 = t6977 * t550;
    let t6979 = t6976 * t6978;
    let t6980 = t1992 * t6979;
    let t6982 = t1998 * t1372;
    let t6983 = t214 * t6982;
    let t6984 = t1985 * t6983;
    let t6987 = t1338 * t2006;
    let t6988 = t6987 * t1352;
    let t6990 = t553 * t6955;
    let t6992 = -t6967 - 0.16449340668482264365e-1 * t6971 - t6975 - 0.82246703342411321825e-2 * t6980 + 0.82246703342411321825e-2 * t6984 + t1332 * t2013 - t1336 * t6988 + t544 * t6990;
    (t6978, t6979, t6980, t6982, t6983, t6984, t6987, t6988, t6990, t6992)
}

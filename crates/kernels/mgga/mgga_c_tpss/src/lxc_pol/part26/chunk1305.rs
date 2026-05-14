//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1305/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1305<F: Float>(t14245: F, t19671: F, t1398: F, t3610: F, t17930: F, t1288: F, t3683: F, t823: F, t51780: F, t21262: F, t60960: F, t13334: F, t1692: F, t1713: F, t17929: F, t19670: F, t19672: F, t19798: F, t19810: F, t21263: F, t2439: F, t6120: F, t61269: F, t64284: F, t69789: F, t69793: F, t69796: F, t69800: F, t69804: F) -> (F, F) {
    let t69807 = t19671 * t14245;
    let t69810 = t3610 * t1398;
    let t69811 = t17930 * t69810;
    let t69817 = t823 * t1288 * t3683;
    let t69820 = t17930 * t51780;
    let t69828 = t60960 * t21262;
    let t69834 = -3.0 * t17929 * t69789 + 6.0 * t69793 * t19672 - 3.0 * t17929 * t69796 - 6.0 * t19670 * t69800 + 6.0 * t17929 * t69804 + 6.0 * t19670 * t69807 - 3.0 * t17929 * t69811 - 3.0 * t64284 * t19810 + 6.0 * t19670 * t69817 - 3.0 * t19670 * t69820 - 3.0 * t61269 * t21263 + t1692 * t1713 * t13334 / 2.0 - 3.0 * t17929 * t69828 + 3.0 * t2439 * t19798 * t6120;
    (t69810, t69834)
}

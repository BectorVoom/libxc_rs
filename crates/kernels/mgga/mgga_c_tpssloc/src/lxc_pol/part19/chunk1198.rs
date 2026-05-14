//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1198/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1198<F: Float>(t2887: F, t10727: F, t10817: F, t10655: F, t10731: F, t10661: F, t2836: F, t2845: F, t10697: F, t2792: F, t912: F, t41654: F, t41642: F, t41646: F, t41651: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F) -> (F, F, F, F, F, F) {
    let t42227 = t2887 * t2887;
    let t42228 = 1.0 / t42227;
    let t42233 = 24.0 * t10817 * t10727;
    let t42235 = 0.1929837539843104208e3 * t10655 * t10731;
    let t42238 = 0.57895126195293126241e3 * t10661 * t2845 * t2836;
    let t42241 = 8.0 * t2792 * t10697 * t912;
    let t42245 = 0.17757530864197530864e0 * t41654;
    let t42253 = 0.10274e0 * t41642 + 0.13698666666666666667e0 * t41646 + 0.41095999999999999998e0 * t41651 + t42245 - 0.45662222222222222221e-1 * t41656 - 0.3044148148148148148e-1 * t41658 + 0.25367901234567901233e-1 * t41660 + 0.22831111111111111111e-1 * t41662 - 0.50735802469135802467e-1 * t41669 - 0.17123333333333333333e-1 * t41673 + 0.9132444444444444444e-1 * t41675;
    (t42228, t42233, t42235, t42238, t42241, t42253)
}

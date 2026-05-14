//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1309/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1309<F: Float>(t3324: F, t3356: F, t43748: F, t43750: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t43819: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43811: F, t43816: F, t43823: F, t43828: F) -> (F, F, F) {
    let t44300 = t3324 * t3356;
    let t44314 = -0.3044148148148148148e-1 * t43748 - 0.25367901234567901233e-1 * t43750 + 0.45662222222222222221e-1 * t43780 + 0.9132444444444444444e-1 * t43782 + 0.9132444444444444444e-1 * t43784 - 0.13698666666666666667e0 * t43786 - 0.22831111111111111111e-1 * t43788 + 0.2283111111111111111e0 * t43794 - 0.41095999999999999999e0 * t43798 + 0.41096e0 * t43802 + 0.17123333333333333333e-1 * t43806;
    let t44320 = 0.17757530864197530864e0 * t43819;
    let t44327 = -0.50735802469135802467e-1 * t43811 + 0.4566222222222222222e-1 * t43727 - 0.13698666666666666667e0 * t43729 + 0.11415555555555555555e0 * t43734 - 0.71030123456790123454e-1 * t43816 + t44320 - 0.41095999999999999998e0 * t43737 - 0.34246666666666666665e-1 * t43823 - 0.4566222222222222222e-1 * t43740 + 0.61644e0 * t43743 + 0.10274e0 * t43828 + 0.13698666666666666667e0 * t43746;
    (t44300, t44314, t44327)
}

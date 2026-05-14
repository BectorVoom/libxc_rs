//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 837/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk837<F: Float>(t77840: F, t14516: F, t8526: F, t2329: F, t71882: F, t76148: F, t76151: F, t76154: F, t76159: F, t71863: F, t71871: F, t71892: F, t76173: F, t76175: F, t76178: F, t76186: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77841 = 0.10227998120342003148e-1 * t77840;
    let t77842 = t14516 * t8526;
    let t77843 = 0.10227998120342003148e-1 * t77842;
    let t77844 = t71882 * t2329;
    let t77845 = 0.13637330827122670864e-1 * t77844;
    let t77846 = 0.40911992481368012596e-1 * t76148;
    let t77848 = 0.40911992481368012595e-1 * t76151;
    let t77849 = 0.5454932330849068346e-1 * t76154;
    let t77850 = 0.40911992481368012595e-1 * t76159;
    let t77851 = 0.18183107769496894486e-1 * t71863;
    let t77852 = 0.36366215538993788972e-1 * t71871;
    let t77853 = 0.27274661654245341729e-1 * t71892;
    let t77860 = 0.20455996240684006296e-1 * t76173;
    let t77863 = 0.40911992481368012592e-1 * t76175;
    let t77864 = 0.20455996240684006296e-1 * t76178;
    let t77868 = 0.20455996240684006298e-1 * t76186;
    (t77841, t77843, t77845, t77846, t77848, t77849, t77850, t77851, t77852, t77853, t77860, t77863, t77864, t77868)
}

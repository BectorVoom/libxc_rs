//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 863/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk863<F: Float>(t71863: F, t71871: F, t71892: F, t76173: F, t76161: F, t76163: F, t76165: F, t76167: F, t76169: F, t76171: F, t77848: F, t77849: F, t77850: F, t76175: F, t76178: F, t76186: F) -> (F, F, F, F) {
    let t77851 = 0.18183107769496894486e-1 * t71863;
    let t77852 = 0.36366215538993788972e-1 * t71871;
    let t77853 = 0.27274661654245341729e-1 * t71892;
    let t77860 = 0.20455996240684006296e-1 * t76173;
    let t77861 = -t77848 + t77849 + t77850 + t77851 + t77852 - t77853 - 0.18637685463734316849e-1 * t76161 + 0.46594213659335792122e-1 * t76163 + 0.93188427318671584245e-2 * t76165 + 0.46594213659335792124e-1 * t76167 - 0.93188427318671584248e-1 * t76169 - 0.15531404553111930708e-1 * t76171 - t77860;
    let t77863 = 0.40911992481368012592e-1 * t76175;
    let t77864 = 0.20455996240684006296e-1 * t76178;
    let t77868 = 0.20455996240684006298e-1 * t76186;
    (t77861, t77863, t77864, t77868)
}

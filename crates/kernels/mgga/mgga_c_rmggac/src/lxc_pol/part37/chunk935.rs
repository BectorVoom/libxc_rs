//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 935/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk935<F: Float>(t76161: F, t76163: F, t76165: F, t76167: F, t76169: F, t77845: F, t77846: F, t77848: F, t77849: F, t77850: F, t77851: F, t77852: F, t77853: F, t305: F, t69146: F, t76171: F, t76180: F, t76182: F, t76184: F, t77860: F, t77863: F, t77864: F, t77868: F, t77869: F, t77870: F, t77873: F, t80398: F) -> (F, F) {
    let t80413 = -t77845 + t77846 - t77848 + t77849 + t77850 + t77851 + t77852 - t77853 - 0.18637685463734316848e-1 * t76161 + 0.46594213659335792121e-1 * t76163 + 0.93188427318671584242e-2 * t76165 + 0.46594213659335792121e-1 * t76167 - 0.93188427318671584242e-1 * t76169;
    let t80421 = -0.15531404553111930707e-1 * t76171 - t77860 + t77863 + t77864 + 0.93188427318671584242e-2 * t76180 - 0.15531404553111930707e-1 * t76182 - 0.62125618212447722828e-2 * t76184 + t77868 - t77869 - t77870 + 0.59871208509319042821e-1 * t305 * t80398 - t77873 - t69146;
    (t80413, t80421)
}

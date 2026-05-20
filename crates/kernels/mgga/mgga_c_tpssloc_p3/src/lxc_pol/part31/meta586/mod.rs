//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1827;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta586<F: Float>(t26426: F, t81046: F, t22690: F, t7732: F, t81195: F, t22832: F, t5234: F, t1336: F, t22759: F, t5252: F, t836: F, t5293: F, t80820: F, t1831: F, t80869: F, t22783: F, t5314: F, t26297: F, t80853: F, t80855: F, t26301: F, t80866: F, t131: F, t6931: F, t9537: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t91078, t91081, t91100, t91113, t91120) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1827::<F>(t26426, t81046, t22690, t7732, t81195, t22832, t5234, t1336, t22759, t5252, t836, t5293, t80820);
        let (t91135, t91137, t91140, t91143, t91149, t91152) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1828::<F>(t1831, t80869, t22783, t5314, t26297, t80853, t80855, t26301, t80866, t131, t6931, t9537);
    (t91078, t91081, t91100, t91113, t91120, t91135, t91137, t91140, t91143, t91149, t91152)
}

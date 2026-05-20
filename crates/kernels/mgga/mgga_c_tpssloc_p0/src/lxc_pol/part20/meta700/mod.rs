//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta700 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta700<F: Float>(t182: F, t54374: F, t39510: F, t39512: F, t39514: F, t39522: F, t39530: F, t39532: F, t39496: F, t39499: F, t39502: F, t39505: F, t39508: F, t39518: F, t39521: F, t39529: F, t39539: F) -> (F, F, F, F, F, F, F, F) {
        let (t54419, t54420, t54421, t54422, t54423, t54424, t54425, t54426) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2668::<F>(t182, t54374, t39510, t39512, t39514, t39522, t39530, t39532, t39496, t39499, t39502, t39505, t39508, t39518, t39521, t39529, t39539);
    (t54419, t54420, t54421, t54422, t54423, t54424, t54425, t54426)
}

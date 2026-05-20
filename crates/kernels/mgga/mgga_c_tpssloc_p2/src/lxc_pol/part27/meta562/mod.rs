//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2005;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta562<F: Float>(t1799: F, t3850: F, t1824: F, t3791: F, t16028: F, t225: F, t1372: F, t5286: F, t3879: F, t16205: F, t562: F, t1834: F) -> (F, F, F, F, F, F, F) {
        let (t54165, t54258, t54825, t54840, t54854, t54883, t54918) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2005::<F>(t1799, t3850, t1824, t3791, t16028, t225, t1372, t5286, t3879, t16205, t562, t1834);
    (t54165, t54258, t54825, t54840, t54854, t54883, t54918)
}

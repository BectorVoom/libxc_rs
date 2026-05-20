//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta208 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta208<F: Float>(t12248: F, t236: F, t240: F, t1336: F, t10022: F, t248: F, t557: F, t555: F, t10027: F, t541: F, t1361: F, t2690: F) -> (F, F, F, F, F, F, F) {
        let (t12289, t12290, t12291, t12328, t12330, t12335, t12344) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk851::<F>(t12248, t236, t240, t1336, t10022, t248, t557, t555, t10027, t541, t1361, t2690);
    (t12289, t12290, t12291, t12328, t12330, t12335, t12344)
}

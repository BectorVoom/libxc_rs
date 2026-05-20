//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1826;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta468<F: Float>(t23394: F, t3175: F, t6704: F, t1922: F, t3010: F, t2776: F, t6690: F, t6689: F, t1945: F, t3020: F, t6768: F, t990: F, t2250: F, t3: F, t1933: F, t368: F, t3068: F, t1058: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23395, t23396, t23399, t23402, t23403, t23408, t23410) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1826::<F>(t23394, t3175, t6704, t1922, t3010, t2776, t6690, t6689, t1945, t3020, t6768, t990);
        let (t23413, t23414, t23417, t23418, t23419) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1827::<F>(t2250, t3, t1933, t368, t3068, t1058, sigma0);
    (t23395, t23396, t23399, t23402, t23403, t23408, t23410, t23413, t23414, t23417, t23418, t23419)
}

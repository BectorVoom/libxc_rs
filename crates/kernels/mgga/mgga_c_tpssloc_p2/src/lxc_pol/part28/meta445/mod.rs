//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1629;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1630;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta445<F: Float>(t776: F, t857: F, t865: F, t23270: F, t22986: F, t25: F, t2749: F, t606: F, t868: F, t2745: F, t2379: F, t28: F, t2752: F) -> (F, F, F, F, F, F, F, F) {
        let (t23272, t23273, t23274, t23296, t23299, t23302, t23781) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1629::<F>(t776, t857, t865, t23270, t22986, t25, t2749, t606, t868, t2745, t2379, t28);
        let t23788 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1630::<F>(t2752, t28);
    (t23272, t23273, t23274, t23296, t23299, t23302, t23781, t23788)
}

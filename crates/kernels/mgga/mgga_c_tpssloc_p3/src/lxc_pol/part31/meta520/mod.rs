//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1728;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1729;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1730;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta520<F: Float>(t2039: F, t6287: F, t2075: F, t5493: F, t1774: F, t7801: F, t19596: F, t2095: F, t1268: F, t1458: F, t19451: F, t27188: F, t28002: F, t28007: F, t28943: F, t28951: F, t28959: F, t4028: F, t7042: F, t7676: F, t20085: F, t24432: F, t28830: F, t23957: F, t28826: F, t26231: F, t26246: F, t26251: F, t26255: F, t26266: F, t26268: F, t28058: F, t28061: F, t28063: F, t28065: F, t28068: F, t28070: F, t28074: F, t28078: F, t28080: F, t24049: F, t24050: F, t24058: F, t24060: F, t24061: F, t26272: F, t26295: F, t28085: F, t28089: F, t28091: F, t28093: F, t28095: F, t28097: F, t28102: F, t28104: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t29211, t29214, t29219, t29222, t29241) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1728::<F>(t2039, t6287, t2075, t5493, t1774, t7801, t19596, t2095, t1268, t1458, t19451, t27188, t28002, t28007, t28943, t28951, t28959, t4028, t7042, t7676);
        let (t29243, t29247, t29252, t29274) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1729::<F>(t20085, t2095, t24432, t28830, t23957, t28826, t26231, t26246, t26251, t26255, t26266, t26268, t28058, t28061, t28063, t28065, t28068, t28070, t28074, t28078, t28080);
        let t29285 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1730::<F>(t24049, t24050, t24058, t24060, t24061, t26272, t26295, t28085, t28089, t28091, t28093, t28095, t28097, t28102, t28104);
    (t29211, t29214, t29219, t29222, t29241, t29243, t29247, t29252, t29274, t29285)
}

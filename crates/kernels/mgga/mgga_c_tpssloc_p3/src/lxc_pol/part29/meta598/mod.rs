//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2029;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2030;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta598<F: Float>(t131: F, t23121: F, t9537: F, t236: F, t81613: F, t23098: F, t22822: F, t281: F, t6589: F, t23124: F, t23076: F, t6597: F, t22690: F, t2379: F, t841: F, t23072: F, t23083: F, t23069: F, t2610: F, t2690: F, t6612: F, t812: F, t831: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t81782, t81783, t81785, t81788, t81789, t81792) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2029::<F>(t131, t23121, t9537, t236, t81613, t23098, t22822, t281, t6589, t23124, t23076, t6597);
        let (t81795, t81797, t81799, t81807, t81808) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2030::<F>(t22690, t2379, t81792, t841, t23072, t23083, t23069, t2610, t2690, t6612, t812, t831);
    (t81782, t81783, t81785, t81788, t81789, t81792, t81795, t81797, t81799, t81807, t81808)
}

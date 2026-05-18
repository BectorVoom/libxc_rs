//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1046/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1046<F: Float>(t52: F, t3966: F, t78: F, t12606: F, t1431: F, t2244: F, t2250: F, t4111: F, t607: F, t771: F, t12958: F, zeta_threshold: F) -> F {
    let t150 = t52 <= zeta_threshold;
    let t12961 = t78 * t3966;
    let t12969 = piecewise3::<f64>(t150, F::new(0.0), -F::new(8.0) / F::new(27.0) * t1431 * t2244 - F::new(4.0) / F::new(9.0) * t12961 * t607 - F::new(2.0) / F::new(9.0) * t4111 * t2250 - F::new(2.0) / F::new(3.0) * t771 * t12606);
    let t12971 = t12958 / F::new(2.0) + t12969 / F::new(2.0);
    t12971
}

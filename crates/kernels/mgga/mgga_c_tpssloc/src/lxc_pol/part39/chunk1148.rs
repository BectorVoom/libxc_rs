//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1148/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1148<F: Float>(t2906: F, t4475: F, t2932: F, t4471: F, t950: F, t1581: F, t1569: F, t2862: F, t10747: F, t10771: F, t10811: F, t10825: F, t10828: F, t14429: F, t14432: F, t14436: F, t14439: F, t14443: F, t14450: F, t14453: F, t2861: F, t2886: F, t2905: F, t2930: F, t4454: F, t4476: F) -> F {
    let t14456 = t4475 * t2906;
    let t14459 = t4471 * t2932;
    let t14460 = t14459 * t950;
    let t14463 = t1581 * t2906;
    let t14466 = t1569 * t2862;
    let t14469 = -F::new(2.0) * t2861 * t14429 - F::new(0.19298375398431042081e3) * t10771 * t14432 + F::new(0.64327917994770140268e2) * t2886 * t14436 + F::new(0.32163958997385070134e2) * t2886 * t14439 + F::new(0.2069040516770936012e4) * t10811 * t14443 - F::new(0.23392894490538584828e1) * t10747 * t4454 + F::new(0.34631718211362927518e2) * t10825 * t4476 - F::new(0.23392894490538584828e1) * t2905 * t14450 - F::new(0.11696447245269292414e1) * t2905 * t14453 - F::new(0.10389515463408878255e3) * t10828 * t14456 + F::new(0.34631718211362927518e2) * t2930 * t14460 + F::new(0.35089341735807877242e1) * t2930 * t14463 + F::new(6.0) * t2886 * t14466;
    t14469
}

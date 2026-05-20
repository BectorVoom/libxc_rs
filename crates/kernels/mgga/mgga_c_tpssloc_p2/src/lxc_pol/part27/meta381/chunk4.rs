//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1569/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1569<F: Float>(t2906: F, t4475: F, t2932: F, t4471: F, t950: F, t1581: F, t1569: F, t2862: F, t10747: F, t10771: F, t10811: F, t10825: F, t10828: F, t14429: F, t14432: F, t14436: F, t14439: F, t14443: F, t14450: F, t14453: F, t2861: F, t2886: F, t2905: F, t2930: F, t4454: F, t4476: F) -> F {
    let t14456 = t4475 * t2906;
    let t14459 = t4471 * t2932;
    let t14460 = t14459 * t950;
    let t14463 = t1581 * t2906;
    let t14466 = t1569 * t2862;
    let t14469 = -F::new(2.0) * t2861 * t14429 - F::cast_from(0.19298375398431042081e3_f64) * t10771 * t14432 + F::cast_from(0.64327917994770140268e2_f64) * t2886 * t14436 + F::cast_from(0.32163958997385070134e2_f64) * t2886 * t14439 + F::cast_from(0.2069040516770936012e4_f64) * t10811 * t14443 - F::cast_from(0.23392894490538584828e1_f64) * t10747 * t4454 + F::cast_from(0.34631718211362927518e2_f64) * t10825 * t4476 - F::cast_from(0.23392894490538584828e1_f64) * t2905 * t14450 - F::cast_from(0.11696447245269292414e1_f64) * t2905 * t14453 - F::cast_from(0.10389515463408878255e3_f64) * t10828 * t14456 + F::cast_from(0.34631718211362927518e2_f64) * t2930 * t14460 + F::cast_from(0.35089341735807877242e1_f64) * t2930 * t14463 + F::new(6.0) * t2886 * t14466;
    t14469
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1309/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1309<F: Float>(t30328: F, t30347: F, t3: F, t112: F, t8283: F, t1458: F, t8189: F, t2199: F, t4072: F, t671: F, t8273: F, t12524: F, t1401: F, t16521: F, t16524: F, t20173: F, t30109: F, t30112: F, t30315: F, t3938: F, t3941: F, t5371: F, t5376: F, t577: F, t8207: F, t8212: F, t8294: F) -> (F, F, F, F, F, F, F) {
    let t30349 = F::new(2.0) * t30328 + F::new(2.0) * t30347;
    let t30350 = t3 * t30349;
    let t30363 = t8283 * t112;
    let t30382 = t8189 * t1458;
    let t30385 = t2199 * t4072;
    let t30390 = t8273 * t671;
    let t30395 = F::new(0.45e1) * t30349 * t577 + F::new(0.135e2) * t30363 * t671 + F::new(0.135e2) * t30109 * t1458 + F::new(27.0) * t30112 * t5376 + F::new(0.135e2) * t8207 * t4072 + F::new(0.135e2) * t16521 * t2199 + F::new(27.0) * t16524 * t8212 + F::new(0.135e2) * t5371 * t8189 + F::new(27.0) * t12524 * t8294 + F::new(27.0) * t20173 * t8294 + F::new(27.0) * t3941 * t30382 + F::new(27.0) * t3941 * t30385 + F::new(0.135e2) * t3938 * t8273 + F::new(27.0) * t3941 * t30390 + F::new(0.135e2) * t1401 * t30315;
    (t30349, t30350, t30363, t30382, t30385, t30390, t30395)
}

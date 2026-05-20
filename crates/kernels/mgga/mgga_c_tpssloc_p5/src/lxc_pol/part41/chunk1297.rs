//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1297/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1297<F: Float>(t109: F, t111715: F, t111751: F, t111772: F, t111805: F, t2205: F, t671: F, t100930: F, t110363: F, t111226: F, t111246: F, t1401: F, t1458: F, t16524: F, t19534: F, t20162: F, t20173: F, t2199: F, t28893: F, t30315: F, t30363: F, t30382: F, t30385: F, t30611: F, t3941: F, t4072: F, t5371: F, t5376: F, t5456: F, t5493: F, t75795: F, t8189: F, t8207: F, t8294: F) -> (F, F) {
    let t110 = F::new(1.0) < t109;
    let t111808 = piecewise3::<F>(t110, F::new(0.0), t111715 + t111751 + t111772 + t111805);
    let t111819 = t2205 * t671;
    let t111842 = F::new(27.0) * t110363 * t5456 + F::new(27.0) * t28893 * t8189 + F::new(0.135e2) * t1401 * t111808 + F::new(54.0) * t111246 * t5376 + F::new(54.0) * t16524 * t30385 + F::new(27.0) * t100930 * t2199 + F::new(0.135e2) * t8207 * t19534 + F::new(27.0) * t111819 * t5456 + F::new(27.0) * t111226 * t1458 + F::new(54.0) * t16524 * t30382 + F::new(27.0) * t20173 * t30611 + F::new(27.0) * t30363 * t4072 + F::new(54.0) * t75795 * t8294 + F::new(27.0) * t5371 * t30315 + F::new(0.135e2) * t20162 * t8189 + F::new(27.0) * t3941 * t8189 * t5493 + F::new(27.0) * t3941 * t2199 * t19534;
    (t111808, t111842)
}

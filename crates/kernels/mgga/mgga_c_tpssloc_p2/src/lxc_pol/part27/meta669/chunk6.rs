//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2369/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2369<F: Float>(t2022: F, t2319: F, t1458: F, t16538: F, t16541: F, t2363: F, t23877: F, t23880: F, t26523: F, t4072: F, t5376: F, t577: F, t671: F, t83980: F, t86642: F, t86646: F, t86647: F, t86651: F, t86653: F, t86655: F, t86656: F, t86660: F, t86668: F, t91792: F, t91799: F, t91802: F) -> F {
    let t91803 = t2022 * t2319;
    let t91806 = t86642 + F::new(0.135e2) * t26523 * t2363 + t86646 + F::new(27.0) * t86647 * t2319 + t86651 + t86653 + t86655 + F::new(27.0) * t86656 * t671 + t86660 + F::new(27.0) * t23877 * t4072 + F::new(54.0) * t23880 * t16538 + F::new(27.0) * t23880 * t16541 + t86668 + F::new(0.45e1) * t91792 * t577 + F::new(54.0) * t83980 * t5376 + t91799 + t91802 + F::new(27.0) * t91803 * t1458;
    t91806
}

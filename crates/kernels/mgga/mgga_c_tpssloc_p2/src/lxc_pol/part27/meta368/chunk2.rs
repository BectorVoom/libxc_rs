//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1518/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1518<F: Float>(t10556: F, t10558: F, t10560: F, t10562: F, t10577: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13600: F, t13601: F, t13603: F, t13613: F) -> F {
    let t13615 = -t10577 - F::new(8.0) / F::new(27.0) * t10556 + F::new(2.0) / F::new(27.0) * t10558 - F::new(2.0) / F::new(9.0) * t10560 + t10562 / F::new(9.0) - F::new(4.0) / F::new(27.0) * t13598 + t13600 - t13601 + t13603 - F::new(10.0) / F::new(27.0) * t13569 + F::new(4.0) / F::new(3.0) * t13572 - F::new(4.0) / F::new(9.0) * t13575 - F::new(2.0) / F::new(9.0) * t13578 - F::new(2.0) * t13581 + F::new(4.0) / F::new(3.0) * t13584 + F::new(2.0) / F::new(3.0) * t13587 - t13613 / F::new(3.0);
    t13615
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2355/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2355<F: Float>(t12734: F, t7461: F, t2314: F, t25980: F, t22574: F, t56120: F, t8643: F, t1845: F, t3719: F, t1874: F, t55962: F, t19456: F, t6525: F) -> (F, F, F, F, F, F) {
    let t91591 = F::new(4.0) * t12734 * t7461;
    let t91593 = F::new(4.0) * t2314 * t25980;
    let t91602 = F::new(3.0) * t22574 * t8643 * t56120;
    let t91603 = t1845 * t3719;
    let t91606 = F::new(3.0) * t22574 * t8643 * t91603;
    let t91608 = F::new(2.0) * t55962 * t1874;
    let t91610 = F::new(4.0) * t19456 * t6525;
    (t91591, t91593, t91602, t91606, t91608, t91610)
}

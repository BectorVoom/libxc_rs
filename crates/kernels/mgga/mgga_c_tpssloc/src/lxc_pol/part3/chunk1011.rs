//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1011/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1011<F: Float>(t2427: F, t4202: F, t9869: F, t2655: F, t4205: F, t12914: F, t12922: F, t12926: F, t12927: F, t12928: F, t12934: F, t12942: F, t12944: F, t12947: F, t9724: F, t9780: F, t9789: F, t9863: F) -> (F, F, F, F) {
    let t13095 = F::new(8.0) * t2427 * t4202;
    let t13096 = F::new(8.0) * t9869;
    let t13098 = F::new(4.0) * t4205 * t2655;
    let t13099 = t9724 + t12914 + t9863 + t9780 + t12922 + t12926 + t12927 - t12928 + t12934 + t12942 + t12944 + t12947 + t13095 - t9789 + t13096 + t13098;
    (t13095, t13096, t13098, t13099)
}

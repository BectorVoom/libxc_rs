//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 935/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk935<F: Float>(t13048: F, t13470: F, t12910: F, t12914: F, t12915: F, t12922: F, t12926: F, t12927: F, t12928: F, t12934: F, t12935: F, t12942: F, t12944: F, t12947: F, t12971: F, t1484: F, t1877: F, t193: F, t202: F, t2522: F, t2523: F, t2745: F, t2749: F, t4255: F, t4307: F, t4314: F, t766: F, t870: F, t9470: F, t9724: F, t9780: F, t9863: F) -> (F,) {
    let t13471 = t13048 + t13470;
    let t13475 = t12910 + t9724 + 12.0 * t4314 * t2523 * t4255 + t12914 + t9863 + t9780 + 2.0 * t1877 * t12915 * t2749 - t1877 * t4307 * t2745 + t12922 + t12926 + t12927 - t12928 - 3.0 * t2522 * t9470 * t1484 + t12934 + 6.0 * t193 * t12935 * t1484 + t12942 + t12944 + t12947 + 3.0 * t193 * t766 * t12971 + t193 * t202 * t13471 * t870;
    (t13475,)
}

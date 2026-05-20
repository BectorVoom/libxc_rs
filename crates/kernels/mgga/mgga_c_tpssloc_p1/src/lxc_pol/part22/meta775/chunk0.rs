//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2650/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2650<F: Float>(t39365: F, t56168: F, t54380: F, t54382: F, t39374: F, t54389: F, t56185: F, t54392: F, t15883: F, t19577: F, t19596: F, t19631: F, t3918: F, t39400: F, t39408: F, t39411: F, t39463: F, t39468: F, t5126: F, t5127: F, t6347: F) -> (F, F, F, F, F, F, F, F, F) {
    let t74040 = F::cast_from(0.56968947174242584612e-3_f64) * t39365;
    let t74041 = F::cast_from(0.35089341735807877242e1_f64) * t56168;
    let t74042 = F::cast_from(0.48796115851357829289e-1_f64) * t54380;
    let t74043 = F::cast_from(0.14447919941302971323e1_f64) * t54382;
    let t74044 = F::cast_from(0.10254018858216406658e4_f64) * t39374;
    let t74046 = F::cast_from(0.17544670867903938621e1_f64) * t54389;
    let t74056 = F::new(24.0) * t56185;
    let t74057 = F::cast_from(0.10526802520742363173e2_f64) * t54392;
    let t74058 = F::new(18.0) * t15883 * t5126 * t6347 - F::new(9.0) * t19577 * t19596 * t3918 + F::new(18.0) * t19631 * t5126 * t5127 - t39400 + t39408 + t39411 + t39463 - t39468 - t74046 - t74056 + t74057;
    (t74040, t74041, t74042, t74043, t74044, t74046, t74056, t74057, t74058)
}

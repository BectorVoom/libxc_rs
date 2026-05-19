//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1342/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1342<F: Float>(t5: F, t85479: F, t85504: F, t85532: F, t85569: F, t112: F, t2319: F, t7263: F, t11968: F, t12492: F, t12504: F, t1266: F, t2114: F, t2165: F, t2167: F, t2314: F, t2320: F, t2323: F, t24543: F, t24545: F, t24932: F, t3652: F, t3929: F, t510: F, t7264: F, t7266: F, t7271: F, t7408: F, t7412: F, t81419: F, t81422: F, t81426: F, t81430: F, t81432: F, t81434: F, t81458: F, t9348: F, t9351: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t85572 = piecewise3::<F>(t8, F::new(0.0), t85479 + t85504 + t85532 + t85569);
    let t85573 = t85572 * t112;
    let t85577 = t7263 * t2319;
    let t85585 = -t81419 - F::new(6.0) * t7266 * t12504 - F::new(6.0) * t9348 * t7271 - F::new(12.0) * t2314 * t24545 - F::new(12.0) * t24932 * t2323 + t81422 - F::new(6.0) * t2320 * t7408 + t81426 - t81430 - t81432 - t81434 - t81458 - F::new(3.0) * t7264 * t3652 - t2114 * t11968 - t85573 * t510 - F::new(3.0) * t24543 * t1266 - F::new(6.0) * t85577 * t510 - F::new(6.0) * t9351 * t2165 + F::new(3.0) * t7412 * t3929 + t2167 * t12492;
    (t85573, t85577, t85585)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2210/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2210<F: Float>(t12606: F, t3: F, t1025: F, t1933: F, t1937: F, t23453: F, t23504: F, t23515: F, t23521: F, t25588: F, t25645: F, t6722: F, t6747: F, t7573: F, t7583: F, t82927: F, t82961: F, t83111: F, t88362: F, t88367: F, t88372: F, t88385: F, t88388: F) -> F {
    let t88391 = t3 * t12606;
    let t88397 = t82961 / F::cast_from(2304.0_f64) - F::cast_from(0.20186378047070195428e-3_f64) * t88362 * t6747 - F::cast_from(0.20186378047070195428e-3_f64) * t88367 * t6747 - F::cast_from(0.10093189023535097714e-3_f64) * t25645 * t23504 - F::cast_from(0.20186378047070195428e-3_f64) * t88372 * t23515 + F::cast_from(0.10093189023535097714e-3_f64) * t88372 * t23521 + F::cast_from(0.72670960969452703541e-2_f64) * t23453 * t7573 * t1937 - F::cast_from(0.16149102437656156342e-2_f64) * t6722 * t25588 * t1937 - t88385 + F::cast_from(0.72670960969452703541e-2_f64) * t83111 * t7583 + t88388 * t1025 / F::cast_from(768.0_f64) + F::cast_from(0.10093189023535097714e-3_f64) * t1933 * t88391 * t1937 + F::cast_from(0.16149102437656156342e-2_f64) * t82927 * t7583;
    t88397
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2648/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2648<F: Float>(t1447: F, t2349: F, t100: F, t12792: F, t12796: F, t12799: F, t12805: F, t19493: F, t19498: F, t19513: F, t19521: F, t19525: F, t21: F, t2248: F, t2341: F, t2342: F, t2350: F, t2354: F, t4049: F, t4059: F, t45460: F, t45496: F, t45717: F, t5396: F, t5468: F, t5480: F, t5484: F, t584: F, t662: F, t9: F, t92: F, t9384: F, t9398: F) -> F {
    let t55491 = t1447 * t2349;
    let t55512 = F::new(10.0) / F::new(9.0) * t92 * t19498 * t2248 - F::new(100.0) / F::new(27.0) * t1447 * t12799 - F::new(10.0) / F::new(27.0) * t100 * t19513 * t2354 + F::new(20.0) / F::new(9.0) * t100 * t2349 * t9 * t21 + F::new(40.0) / F::new(81.0) * t92 * t45496 * t5468 * t2342 + F::new(20.0) / F::new(9.0) * t92 * t4049 * t584 - F::new(10.0) / F::new(27.0) * t92 * t9384 * t5396 * t2342 + F::new(100.0) / F::new(81.0) * t1447 * t12792 - F::new(50.0) / F::new(3.0) * t1447 * t12805 + F::new(40.0) / F::new(81.0) * t100 * t45460 * t5480 * t2350 - F::new(200.0) / F::new(27.0) * t45717 * t19493 + F::new(200.0) / F::new(27.0) * t55491 * t12796 - F::new(20.0) / F::new(9.0) * t100 * t4059 * t584 - F::new(10.0) / F::new(27.0) * t100 * t9398 * t5484 * t2350 + F::new(20.0) / F::new(9.0) * t92 * t2341 * t9 * t21 + F::new(20.0) / F::new(9.0) * t100 * t2349 * t19525 * t662 + F::new(10.0) / F::new(9.0) * t100 * t19521 * t2354;
    t55512
}

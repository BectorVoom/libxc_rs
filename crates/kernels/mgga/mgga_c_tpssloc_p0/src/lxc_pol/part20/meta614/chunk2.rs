//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2206/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2206<F: Float>(t100: F, t4063: F, t591: F, t4053: F, t92: F, t103: F, t12771: F, t12781: F, t12784: F, t1444: F, t1445: F, t1447: F, t1449: F, t16: F, t2341: F, t2349: F, t4059: F, t45460: F, t45496: F, t584: F, t657: F, t659: F, t662: F, t9374: F, t9385: F, t9399: F, t9400: F, t9407: F, t9408: F, t95: F) -> F {
    let t45751 = F::cast_from(20.0_f64) * t100 * t4063 * t591;
    let t45762 = F::cast_from(20.0_f64) * t92 * t4053 * t591;
    let t45775 = F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t100 * t4059 * t9407 - F::cast_from(2200.0_f64) / F::cast_from(81.0_f64) * t9374 * t1445 - F::cast_from(25.0_f64) / F::cast_from(3.0_f64) * t657 * t12781 - F::cast_from(10.0_f64) * t92 * t95 * t16 + F::cast_from(50.0_f64) / F::cast_from(81.0_f64) * t1447 * t9400 + F::cast_from(10.0_f64) * t100 * t103 * t16 - F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t1447 * t9408 - t45751 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t92 * t45496 * t1444 * t9385 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t92 * t2341 * t584 * t659 + t45762 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t100 * t45460 * t1449 * t9399 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t100 * t2349 * t584 * t662 + F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t657 * t12771 + F::cast_from(25.0_f64) * t657 * t12784;
    t45775
}

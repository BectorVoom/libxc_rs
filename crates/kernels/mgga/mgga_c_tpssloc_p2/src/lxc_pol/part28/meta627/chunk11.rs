//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1966/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1966<F: Float>(t2094: F, t40611: F, t12461: F, t7216: F, t1266: F, t12734: F, t1983: F, t22574: F, t2314: F, t2323: F, t23857: F, t23933: F, t24169: F, t24433: F, t24995: F, t25988: F, t26161: F, t26163: F, t26870: F, t26902: F, t26906: F, t27147: F, t27170: F, t27171: F, t27180: F, t27188: F, t32193: F, t34711: F, t4028: F, t4034: F, t510: F, t5308: F, t652: F, t671: F, t6876: F, t7685: F, t7806: F, t7940: F, t91655: F, t91687: F, t92128: F) -> F {
    let t92169 = t2094 * t40611;
    let t92200 = t7216 * t12461;
    let t92210 = -F::cast_from(6.0_f64) * t22574 * t32193 * t25988 - F::cast_from(4.0_f64) * t12734 * t7806 - F::cast_from(4.0_f64) * t2314 * t27180 - F::cast_from(6.0_f64) * t26161 * t92169 * t91687 - F::cast_from(6.0_f64) * t91655 * t24433 - F::cast_from(4.0_f64) * t4028 * t23933 - F::cast_from(4.0_f64) * t2314 * t27147 - F::cast_from(4.0_f64) * t652 * t26870 * t671 + F::cast_from(2.0_f64) * t7685 * t24169 - F::cast_from(2.0_f64) * t6876 * t26902 - F::cast_from(2.0_f64) * t652 * t510 * t92128 - F::cast_from(4.0_f64) * t27188 * t2323 - F::cast_from(4.0_f64) * t2314 * t27171 - F::cast_from(4.0_f64) * t4034 * t27171 - F::cast_from(4.0_f64) * t652 * t1266 * t27170 + F::cast_from(6.0_f64) * t6876 * t26906 + F::cast_from(4.0_f64) * t26161 * t92200 * t26163 + F::cast_from(12.0_f64) * t24995 * t34711 * t5308 + F::cast_from(2.0_f64) * t1983 * t7940 * t23857;
    t92210
}

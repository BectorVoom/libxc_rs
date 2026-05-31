//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2981/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2981<F: Float>(t10422: F, t17648: F, t3070: F, t10214: F, t1031: F, t17701: F, t17877: F, t18036: F, t2979: F, t378: F, t42508: F, t42541: F, t49799: F, t49801: F, t49808: F, t49810: F, t49818: F, t49820: F, t59668: F, t59672: F, t59696: F, t59725: F, t59742: F, t973: F, t977: F) -> F {
    let t62234 = t3070 * t10422 * t17648;
    let t62258 = t49799 / F::cast_from(3456.0_f64) + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t49801 + t42541 * t18036 / F::cast_from(1152.0_f64) + t42508 * t17701 / F::cast_from(432.0_f64) - t62234 / F::cast_from(1728.0_f64) - t973 * t977 * t59696 / F::cast_from(144.0_f64) - t973 * t2979 * t59742 / F::cast_from(36.0_f64) + t973 * t2979 * t59668 / F::cast_from(108.0_f64) + t973 * t2979 * t59672 / F::cast_from(216.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t973 * t10214 * t59725 - t49808 / F::cast_from(3456.0_f64) - t17877 * t1031 * t378 / F::cast_from(288.0_f64) + t49810 / F::cast_from(3456.0_f64) - t49818 / F::cast_from(3456.0_f64) + t49820 / F::cast_from(2304.0_f64);
    t62258
}

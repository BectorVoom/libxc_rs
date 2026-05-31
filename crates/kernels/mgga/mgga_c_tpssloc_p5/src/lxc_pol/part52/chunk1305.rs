//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1305/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1305<F: Float>(t113135: F, t118376: F, t118381: F, t118436: F, t118465: F, t118949: F, t119737: F, t119743: F, t119746: F, t119755: F, t119763: F, t119766: F, t119780: F, t1649: F, t1877: F, t22959: F, t23290: F, t2522: F, t25372: F, t25892: F, t25901: F, t25905: F, t25928: F, t25934: F, t25938: F, t25945: F, t28: F, t30753: F, t30757: F, t30770: F, t32886: F, t33065: F, t6670: F, t6841: F, t8366: F) -> F {
    let t119783 = -t1877 * t30757 * t25934 / F::cast_from(2.0_f64) - t1877 * t6670 * t119737 + t1877 * t30770 * t25945 + t118436 * t25928 + F::cast_from(2.0_f64) * t25372 * t119743 - t1877 * t6670 * t119746 + t1877 * t30753 * t1649 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t32886 * t6841 - F::cast_from(3.0_f64) * t118376 * t119755 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8366 * t25901 + F::cast_from(3.0_f64) * t118381 * t25892 + F::cast_from(3.0_f64) * t113135 * t119763 - t118465 - t1877 * t6670 * t119766 - t1877 * t23290 * t33065 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8366 * t25905 + t1877 * t118949 * t28 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8366 * t25938 - F::cast_from(3.0_f64) * t22959 * t119780;
    t119783
}

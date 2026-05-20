//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1365/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1365<F: Float>(t115876: F, t33564: F, t31688: F, t33572: F, t115837: F, t115846: F, t115853: F, t115860: F, t115866: F, t115877: F, t115891: F, t115894: F, t115895: F, t119888: F, t119897: F, t119913: F, t119931: F, t121024: F, t121029: F, t121032: F, t121040: F, t121044: F, t121050: F, t121055: F, t121058: F, t2240: F, t31675: F, t31681: F, t31684: F, t33568: F, t39063: F, t63: F) -> F {
    let t121064 = t115876 * t33564;
    let t121066 = t31688 * t33572;
    let t121072 = F::new(10.0) / F::new(27.0) * t115837 - F::new(35.0) / F::new(12.0) * t39063 * t115894 * t121024 + F::new(5.0) / F::new(18.0) * t31681 * t119897 - F::new(20.0) / F::new(27.0) * t121029 + F::new(5.0) / F::new(6.0) * t115895 * t121032 + F::new(5.0) / F::new(18.0) * t115891 * t33568 + F::new(5.0) / F::new(18.0) * t31681 * t119888 + F::new(5.0) / F::new(18.0) * t31681 * t121040 + F::new(5.0) / F::new(18.0) * t31681 * t121044 - F::new(5.0) / F::new(9.0) * t2240 * t119931 * t63 * t121050 + F::new(5.0) / F::new(6.0) * t115895 * t121055 + F::new(5.0) / F::new(18.0) * t121058 * t31684 + F::new(5.0) / F::new(27.0) * t115846 + F::new(5.0) / F::new(27.0) * t115853 - t115860 - F::new(10.0) / F::new(9.0) * t115877 - F::new(10.0) / F::new(9.0) * t121064 + F::new(10.0) / F::new(27.0) * t121066 + F::new(5.0) / F::new(12.0) * t115866 * t33564 + F::new(5.0) / F::new(12.0) * t31675 * t119913;
    t121072
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2639/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2639<F: Float>(t28: F, t265: F, t504: F, t47655: F, t51129: F, t51803: F, t51825: F, t51826: F, t51836: F, t51867: F, t51885: F, t53735: F, t10150: F, t1081: F, t11122: F, t11957: F, t1260: F, t12606: F, t13493: F, t1409: F, t1534: F, t15844: F, t1649: F, t1768: F, t2250: F, t3231: F, t3644: F, t3966: F, t4324: F, t45872: F, t47668: F, t47670: F, t47672: F, t47674: F, t47676: F, t506: F, t5099: F, t52: F, t607: F, t9258: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t53739 = piecewise3::<F>(t505, t51129 + t51803 + t51825 + t51826 + t51836 + t51867 + t51885 + t53735, t47655);
    let t53757 = piecewise3::<F>(t401, t47655 * t28 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t13493 * t1081 + F::new(3.0) / F::new(2.0) * t4324 * t3231 + t1534 * t11122 / F::new(2.0) + t10150 * t1649 / F::new(2.0) - t47668 - t47670 + t47672 + t47674 - t47676, t53739 * t52 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t15844 * t607 - F::new(3.0) / F::new(2.0) * t5099 * t2250 - t1768 * t9258 / F::new(2.0) - t11957 * t1409 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t3644 * t3966 - F::new(3.0) / F::new(2.0) * t1260 * t12606 - t506 * t45872 / F::new(2.0));
    t53757
}

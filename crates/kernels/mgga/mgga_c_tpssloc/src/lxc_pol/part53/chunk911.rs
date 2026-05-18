//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 911/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk911<F: Float>(t5161: F, t8804: F, t1842: F, t8800: F, t3887: F, t2091: F, t7936: F, t12021: F, t8793: F, t1807: F, t8788: F, t32139: F, t32141: F, t32145: F, t32712: F, t32715: F, t32718: F, t32722: F, t32724: F) -> (F, F, F, F, F, F) {
    let t33793 = t8804 * t5161;
    let t33797 = t8800 * t1842;
    let t33798 = t3887 * t33797;
    let t33804 = t3887 * t2091 * t7936;
    let t33810 = t12021 * t8793 * t1842;
    let t33815 = t1807 * t8788;
    let t33822 = -t32139 - F::new(0.19378922925187387609e-1) * t32712 - t32141 - F::new(0.32298204875312312682e-2) * t32715 + t32718 / F::new(384.0) - t32722 / F::new(384.0) - t32145 - t32724 / F::new(96.0);
    (t33793, t33798, t33804, t33810, t33815, t33822)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1028/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1028<F: Float>(t1081: F, t116481: F, t119691: F, t119713: F, t123418: F, t123719: F, t123745: F, t123757: F, t123764: F, t1877: F, t23788: F, t24191: F, t24339: F, t2522: F, t25901: F, t25905: F, t25927: F, t25928: F, t25930: F, t25934: F, t25938: F, t25945: F, t26739: F, t26756: F, t28: F, t32030: F, t32034: F, t32047: F, t33991: F, t34052: F, t6841: F, t6848: F, t7114: F, t7649: F, t7844: F, t8744: F) -> F {
    let t123938 = F::new(3.0) / F::new(2.0) * t2522 * t8744 * t25901 - t1877 * t123719 * t6848 / F::new(2.0) - t1877 * t24339 * t34052 - t1877 * t7114 * t28 * t26739 + t1877 * t32047 * t25945 + F::new(3.0) / F::new(2.0) * t2522 * t8744 * t25938 - F::new(3.0) * t123757 * t119691 + F::new(3.0) * t116481 * t119713 - F::new(3.0) * t24191 * t23788 * t123745 + F::new(3.0) / F::new(2.0) * t2522 * t32030 * t7649 + F::new(2.0) * t26756 * t25927 * t123418 + t123764 * t25928 + t1877 * t33991 * t1081 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2522 * t33991 * t6841 - t1877 * t32034 * t25945 / F::new(2.0) - t1877 * t7114 * t1081 * t7844 + F::new(3.0) / F::new(2.0) * t2522 * t8744 * t25905 + t1877 * t32047 * t25930 + t1877 * t32047 * t25934;
    t123938
}

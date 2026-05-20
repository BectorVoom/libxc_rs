//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1301/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1301<F: Float>(t10756: F, t10806: F, t10813: F, t10814: F, t10828: F, t10829: F, t2856: F, t2889: F, t2905: F, t2930: F, t2932: F, t311: F, t41733: F, t41827: F, t41987: F, t42123: F, t42128: F, t42145: F, t42148: F, t42149: F, t42154: F, t42172: F, t42187: F, t42203: F, t42218: F, t42226: F, t42228: F, t42233: F, t42235: F, t42238: F, t42241: F, t42253: F, t42266: F, t924: F, t932: F, t951: F) -> F {
    let t42270 = F::cast_from(0.1929837539843104208e3_f64) * t42123 * t2889 + F::new(4.0) * t2856 * t10806 - F::cast_from(0.4155806185363551302e3_f64) * t42128 * t10829 + F::cast_from(0.6233709278045326953e3_f64) * t10756 * t41827 * t2932 - F::cast_from(0.14035736694323150897e2_f64) * t10828 * t41827 * t951 - F::cast_from(0.35089341735807877242e1_f64) * t2905 * t41733 * t951 + F::cast_from(0.51947577317044391277e2_f64) * t2930 * t41733 * t2932 + t42145 - t42148 + F::cast_from(0.82761620670837440481e4_f64) * t42149 * t10814 - F::cast_from(0.24828486201251232145e5_f64) * t42154 * t41987 * t10813 + F::new(1.0) * t924 * (t42172 + t42187 + t42203 + t42218) * t932 + F::cast_from(0.19964560303604640732e6_f64) * t42226 * t41987 * t42228 + t42233 - t42235 + t42238 + t42241 - F::new(0.310907e-1) * (t42253 + t42266) * t311;
    t42270
}

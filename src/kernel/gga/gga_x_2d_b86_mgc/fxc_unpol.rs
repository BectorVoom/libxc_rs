//! GGA_X_2D_B86_MGC fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b86_mgc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_PI, M_SQRT2};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_2d_b86_mgc_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = f64::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = f64::sqrt(zeta_threshold);
        let t13 = f64::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t16 = 1.0 / t3 * t15;
        let t17 = M_SQRT2;
        let t18 = f64::sqrt(rho[ip]);
        let t19 = t17 * t18;
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t22 = 1.0 / t21;
        let t23 = sigma[ip] * t22;
        let t25 = 1.0 + 0.16646e-1 * t23;
        let t26 = pow_1_4(t25);
        let t27 = t26 * t26;
        let t28 = t27 * t26;
        let t29 = 1.0 / t28;
        let t32 = 1.0 + 0.4409422067590197497e-2 * t23 * t29;
        let t36 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t32);
        let tzk0 = 2.0 * t36;
        zk[ip] += tzk0;
        let t38 = t17 / t18;
        let t42 = t20 * t20;
        let t43 = 1.0 / t42;
        let t47 = sigma[ip] * sigma[ip];
        let t48 = t42 * t21;
        let t49 = 1.0 / t48;
        let t52 = 1.0 / t28 / t25;
        let t55 = -0.13228266202770592491e-1 * sigma[ip] * t43 * t29 + 0.16514828940848946195e-3 * t47 * t49 * t52;
        let t60 = piecewise3(t2, 0.0, -t16 * t38 * t32 / 3.0 - 2.0 / 3.0 * t16 * t19 * t55);
        let tvrho0 = 2.0 * rho[ip] * t60 + 2.0 * t36;
        vrho[ip] += tvrho0;
        let t65 = t42 * t20;
        let t66 = 1.0 / t65;
        let t67 = sigma[ip] * t66;
        let t70 = 0.4409422067590197497e-2 * t22 * t29 - 0.55049429802829820651e-4 * t67 * t52;
        let t74 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t70);
        let tvsigma0 = 2.0 * rho[ip] * t74;
        vsigma[ip] += tvsigma0;
        let t79 = t17 / t18 / rho[ip];
        let t86 = t42 * rho[ip];
        let t87 = 1.0 / t86;
        let t91 = t42 * t42;
        let t92 = 1.0 / t91;
        let t96 = t47 * sigma[ip];
        let t98 = 1.0 / t91 / t21;
        let t100 = t25 * t25;
        let t102 = 1.0 / t28 / t100;
        let t105 = 0.52913064811082369964e-1 * sigma[ip] * t87 * t29 - 0.16514828940848946195e-2 * t47 * t92 * t52 + 0.14432556733842006814e-4 * t96 * t98 * t102;
        let t110 = piecewise3(t2, 0.0, t16 * t79 * t32 / 6.0 - 2.0 / 3.0 * t16 * t38 * t55 - 2.0 / 3.0 * t16 * t19 * t105);
        let tv2rho20 = 2.0 * rho[ip] * t110 + 4.0 * t60;
        v2rho2[ip] += tv2rho20;
        let t118 = t49 * t52;
        let t122 = 1.0 / t91 / t20;
        let t123 = t47 * t122;
        let t126 = -0.13228266202770592491e-1 * t43 * t29 + 0.49544486822546838586e-3 * t118 * sigma[ip] - 0.48108522446140022714e-5 * t123 * t102;
        let t131 = piecewise3(t2, 0.0, -t16 * t38 * t70 / 3.0 - 2.0 / 3.0 * t16 * t19 * t126);
        let tv2rhosigma0 = 2.0 * rho[ip] * t131 + 2.0 * t74;
        v2rhosigma[ip] += tv2rhosigma0;
        let t137 = 1.0 / t91 / rho[ip];
        let t141 = -0.1100988596056596413e-3 * t66 * t52 + 0.16036174148713340905e-5 * sigma[ip] * t137 * t102;
        let t145 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t141);
        let tv2sigma20 = 2.0 * rho[ip] * t145;
        v2sigma2[ip] += tv2sigma20;
    }
}

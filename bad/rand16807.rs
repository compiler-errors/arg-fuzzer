
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16807(_: S6, _: S4, _: S3, _: S6) {}
        
        fn test16807() { foo16807(S2, S7, S6, S6, S0, S2); }
    

        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16621(_: S3, _: S4, _: S5, _: S6) {}
        
        fn test16621() { foo16621(S2, S7, S0, S3, S0, S0, S7); }
    
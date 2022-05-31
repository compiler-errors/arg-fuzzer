
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16219(_: S0, _: S0, _: S4, _: S5, _: S6) {}
        
        fn test16219() { foo16219(S2, S5, S1, S7, S4, S3, S8); }
    
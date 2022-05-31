
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6851(_: S2, _: S4, _: S6) {}
        
        fn test6851() { foo6851(S4, S0, S7, S2, S3, S0, S4); }
    
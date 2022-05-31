
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6871(_: S4, _: S7) {}
        
        fn test6871() { foo6871(S1, S2, S4, S5, S6); }
    
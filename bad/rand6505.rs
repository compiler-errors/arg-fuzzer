
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6505(_: S4, _: S5, _: S7) {}
        
        fn test6505() { foo6505(S7, S7, S0, S5, S5, S4); }
    
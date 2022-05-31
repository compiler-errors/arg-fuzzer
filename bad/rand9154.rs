
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9154(_: S4, _: S0, _: S7) {}
        
        fn test9154() { foo9154(S5, S0, S5, S7, S7, S5, S4); }
    
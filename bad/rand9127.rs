
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9127(_: S5, _: S4, _: S7, _: S6, _: S2, _: S3, _: S1, _: S8) {}
        
        fn test9127() { foo9127(S0, S6, S5, S1, S7); }
    
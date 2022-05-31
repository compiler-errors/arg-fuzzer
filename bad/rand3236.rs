
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3236(_: S4, _: S5, _: S8) {}
        
        fn test3236() { foo3236(S1, S2, S4, S7); }
    
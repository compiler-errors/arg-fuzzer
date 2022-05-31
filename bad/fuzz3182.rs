
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3182(_: S5, _: S6) {}
        
        fn test3182() { foo3182(S1, S2, S5, S7, S8); }
    

        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9471(_: S1, _: S6, _: S8) {}
        
        fn test9471() { foo9471(S2, S4, S5, S6, S7); }
    
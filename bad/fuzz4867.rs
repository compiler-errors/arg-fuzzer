
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4867(_: S2, _: S6, _: S7) {}
        
        fn test4867() { foo4867(S2, S4, S6, S8); }
    
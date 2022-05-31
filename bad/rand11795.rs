
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11795(_: S2, _: S3, _: S4, _: S5) {}
        
        fn test11795() { foo11795(S7, S0, S4, S2, S7); }
    
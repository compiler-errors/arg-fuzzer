
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1793(_: S2, _: S7) {}
        
        fn test1793() { foo1793(S1, S2, S4, S5, S6); }
    